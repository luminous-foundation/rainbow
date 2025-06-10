use std::{collections::HashMap, fs, mem, path::PathBuf, process::Command, rc::Rc};
use common::{FFIResult, FFIString, ModuleType, Program};
use libloading::{Library, Symbol};

type GetType = unsafe extern "C" fn() -> ModuleType;
type ProcessBeginning = unsafe extern "C" fn(*mut u8, usize) -> FFIResult<Program>;
type ProcessMiddle = unsafe extern "C" fn(Program) -> FFIResult<Program>;
type ProcessEnd = unsafe extern "C" fn(Program);

pub struct ModuleHandler {
    module_dir: PathBuf,
    logging: bool,
    rebuild: bool,

    library_cache: HashMap<String, Rc<Library>>
}

impl ModuleHandler {
    pub fn new(module_dir: PathBuf, logging: bool, rebuild: bool) -> ModuleHandler {
        ModuleHandler { module_dir, logging, rebuild, library_cache: HashMap::new() }
    }

    pub fn call_beginning(&mut self, name: &String, mut input: Vec<u8>) -> Result<Program, String> {
        println!("executing beginning module {name}");

        self.check_module_type(name, ModuleType::BEGINNING)?;

        let module = self.get_module(name)?;
        let symbol = Self::get_symbol::<ProcessBeginning>(name, &module, b"process")?;
        
        let res = unsafe { symbol(input.as_mut_ptr(), input.len()) };

        mem::forget(input);

        match res {
            FFIResult::Ok(v) => Ok(v),
            FFIResult::Error(e) => unsafe { Err(FFIString::to_string(e)) }
        }
    }

    pub fn call_middle(&mut self, name: &String, input: Program) -> Result<Program, String> {
        println!("executing middle module {name}");

        self.check_module_type(name, ModuleType::MIDDLE)?;

        let module = self.get_module(name)?;
        let symbol = Self::get_symbol::<ProcessMiddle>(name, &module, b"process")?;
        
        let res = unsafe { symbol(input) };

        match res {
            FFIResult::Ok(v) => Ok(v),
            FFIResult::Error(e) => unsafe { Err(FFIString::to_string(e)) }
        }
    }

    pub fn call_end(&mut self, name: &String, input: Program) -> Result<(), String> {
        println!("executing end module {name}");

        self.check_module_type(name, ModuleType::END)?;

        let module = self.get_module(name)?;
        let symbol = Self::get_symbol::<ProcessEnd>(name, &module, b"process")?;
        unsafe { symbol(input) };
        Ok(())
    }

    fn get_module(&mut self, name: &String) -> Result<Rc<Library>, String> {
        if let Some(lib) = self.library_cache.get(name) {
            return Ok(lib.clone());
        } else {
            let extension = if cfg!(unix) {
                ".so"
            } else {
                ".dll"
            };

            let folder = if let Ok(str) = fs::canonicalize(self.module_dir.clone()) {
                if let Some(str) = str.to_str() {
                    str.to_string() + "/" + name.as_str()
                } else {
                    return Err(format!("Could not load module `{name}` (failed to convert canonicalized path to string)"))
                }
            } else {
                return Err(format!("Could not load module `{name}` (failed to canonicalize path)"));
            };

            if !fs::exists(folder.clone()).unwrap_or(false) {
                return Err(format!("Module `{name}` could not be found! (given folder: {folder})"));
            }

            let path = folder.clone() + "/lib" + name.as_str() + extension;

            if !fs::exists(path.clone()).unwrap_or(false) {
                if self.logging {
                    println!("No module found. Running build script...");
                }

                self.run_script(&folder, &"build".to_string())?;
            } else if self.rebuild {
                if self.logging {
                    println!("Rebuild is enabled, running build script...");
                }
                
                self.run_script(&folder, &"build".to_string())?;
            }

            let lib = unsafe { Library::new(path) };
            if let Ok(lib) = lib {
                let lib = Rc::new(lib);
                self.library_cache.insert(name.clone(), lib.clone());
                return Ok(lib);
            } else if let Err(err) = lib {
                return Err(format!("Could not load module `{name}` (failed to load library with libloading, given error: `{err})`"));
            } else {
                unreachable!();
            }
        }
    }

    fn check_module_type(&mut self, name: &String, expected: ModuleType) -> Result<(), String> {
        let module = self.get_module(name)?;

        unsafe {
            let get_type = Self::get_symbol::<GetType>(name, &module, b"getType")?;
            if get_type() != expected {
                Err(format!("Module `{name}` had invalid module type `{:?}`, expected `{expected:?}`.\nAre you sure it's in the right place in the pipeline?", get_type()))
            } else {
                Ok(())
            }
        }
    }

    fn get_symbol<'a, T>(module_name: &String, module: &'a Library, name: &'a [u8]) -> Result<Symbol<'a, T>, String> {
        let symbol = unsafe { module.get(name) };

        if let Ok(sym) = symbol {
            return Ok(sym);
        } else if let Err(err) = symbol {
            return Err(format!("Failed to get symbol {} from module {module_name} (given error: {err})", String::from_utf8_lossy(name)));
        } else {
            unreachable!();
        }
    }

    fn run_script(&self, path: &String, name: &String) -> Result<(), String> {
        let output;
        if cfg!(unix) {
            output = Command::new("sh").arg(name.clone() + ".sh").current_dir(path).output();
        } else {
            output = Command::new("cmd").args(&["/C", &(name.clone() + ".bat")]).output();
        }

        if let Err(err) = output {
            return Err(format!("Failed to run build script `{path}` (given error: `{err}`)"));
        } else if let Ok(out) = output {
            if (self.logging && !out.status.success()) || out.stderr.len() > 0 {
                if out.stdout.len() > 0 {
                    println!("stdout:\n{}", String::from_utf8_lossy(&out.stdout));
                }
                if out.stderr.len() > 0 {
                    println!("stderr:\n{}", String::from_utf8_lossy(&out.stderr));

                    return Err(format!("Rebuild failed (build script has stderr output)"));
                }
                println!("{}", out.status);
            }
        }

        Ok(())
    }
}

pub struct Pipeline {
    pub modules: Vec<String>,

    module_handler: ModuleHandler
}

impl Pipeline {
    pub fn new(module_dir: PathBuf, modules: Vec<String>, logging: bool, rebuild: bool) -> Pipeline {
        Pipeline { modules, module_handler: ModuleHandler::new(module_dir, logging, rebuild) }
    }

    pub fn execute_bytes(&mut self, input: Vec<u8>) -> Result<(), String> {
        let mut program = self.module_handler.call_beginning(&self.modules[0], input)?;

        for module in &self.modules[1..self.modules.len()-1] {
            program = self.module_handler.call_middle(module, program)?;
        }

        self.module_handler.call_end(&self.modules[self.modules.len()-1], program)?;

        Ok(())
    }

    pub fn execute_program(&mut self, mut program: Program) -> Result<(), String> {
        for module in &self.modules[0..self.modules.len()-1] {
            program = self.module_handler.call_middle(module, program)?;
        }

        self.module_handler.call_end(&self.modules[self.modules.len()-1], program)?;

        Ok(())
    }
}
