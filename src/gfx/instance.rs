use glfw;

pub struct Instance {
    glfw_instance: glfw::Glfw,
}

impl Instance {
    pub fn new() -> Instance {
        Instance { glfw_instance: glfw::Glfw {} }
    }

    pub fn init(&mut self) -> Result<(), String> {

        self.glfw_instance = try!(glfw::init(glfw::FAIL_ON_ERRORS).map_err(|err| {
            match err {
                glfw::InitError::AlreadyInitialized => String::from("Already initialized"),
                glfw::InitError::Internal => String::from("Internal error"),
            }
        }));

        if !self.glfw_instance.vulkan_supported() {
            return Err(String::from("Vulkan is not supported"));
        }

        Ok(())
    }

    pub fn close(&mut self) -> Result<(), String> {
        Ok(())
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        self.close().expect("Couldn't close the gfx module properly!");
    }
}
