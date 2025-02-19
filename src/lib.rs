use std::{arch::x86_64::_mm_floor_sd, fs, path::PathBuf, process::Command};
use common::{ModuleType, Program};
use libloading::{Library, Symbol};

type GetType = unsafe extern "C" fn() -> ModuleType;

pub struct ModuleHandler {
    pub module_dir: PathBuf,
    pub logging: bool,
}

impl ModuleHandler {
    pub fn new(module_dir: PathBuf, logging: bool) -> ModuleHandler {
        ModuleHandler { module_dir, logging }
    }

    pub fn call_beginning(&self, name: &String, input: Vec<u8>) -> Result<Program, String> {
        println!("executing beginning module {name}");

        self.check_module_type(name, ModuleType::BEGINNING)?;

        Ok(Program { })
    }

    pub fn call_middle(&self, name: &String, input: Program) -> Result<Program, String> {
        println!("executing middle module {name}");

        self.check_module_type(name, ModuleType::MIDDLE)?;

        Ok(Program { })
    }

    pub fn call_end(&self, name: &String) -> Result<(), String> {
        println!("executing end module {name}");

        self.check_module_type(name, ModuleType::END)?;

        Ok(())
    }

    fn get_module(&self, name: &String) -> Result<Library, String> {
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
        }

        let lib = unsafe { Library::new(path) };
        if let Ok(lib) = lib {
            return Ok(lib);
        } else if let Err(err) = lib {
            return Err(format!("Could not load module `{name}` (failed to load library with libloading, given error: `{err})`"));
        } else {
            unreachable!();
        }
    }

    fn check_module_type(&self, name: &String, expected: ModuleType) -> Result<(), String> {
        let module = self.get_module(name)?;

        unsafe {
            let get_type: Symbol<GetType> = module.get(b"getType").unwrap();
            if get_type() != expected {
                Err(format!("Module `{name}` had invalid module type `{:?}`, expected `{expected:?}`.\nAre you sure it's in the right place in the pipeline?", get_type()))
            } else {
                Ok(())
            }
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
            if self.logging {
                if out.stdout.len() > 0 {
                    println!("stdout:\n{}", String::from_utf8_lossy(&out.stdout));
                }
                if out.stderr.len() > 0 {
                    println!("stderr:\n{}", String::from_utf8_lossy(&out.stderr));
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
    pub fn new(module_dir: PathBuf, modules: Vec<String>, logging: bool) -> Pipeline {
        Pipeline { modules, module_handler: ModuleHandler::new(module_dir, logging) }
    }

    pub fn execute_bytes(&self, input: Vec<u8>) -> Result<(), String> {
        let mut program = self.module_handler.call_beginning(&self.modules[0], input)?;

        for module in &self.modules[1..self.modules.len()-1] {
            program = self.module_handler.call_middle(module, program)?;
        }

        self.module_handler.call_end(&self.modules[self.modules.len()-1])?;

        Ok(())
    }

    pub fn execute_program(&self, mut program: Program) -> Result<(), String> {
        for module in &self.modules[0..self.modules.len()-1] {
            program = self.module_handler.call_middle(module, program)?;
        }

        self.module_handler.call_end(&self.modules[self.modules.len()-1])?;

        Ok(())
    }
}

type ModuleFunc = unsafe extern "C" fn(u32, u32) -> u32;

pub fn call_module(path: PathBuf, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        let module = Library::new(path).unwrap();

        let module_func: Symbol<ModuleFunc> = module.get(b"test").unwrap();

        return module_func(arg1, arg2);
    }
}
