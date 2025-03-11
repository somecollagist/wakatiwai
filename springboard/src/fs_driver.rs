use crate::*;

impl FSDriver {
    pub fn name(&self) -> &CStr16 {
        &self.0.name
    }

    pub fn load(&mut self) -> Status {
        self.0.load()
    }

    pub fn unload(&mut self) -> Status {
        self.0.unload()
    }

    pub fn invoke(&mut self) -> Status {
        self.0.invoke()
    }
}