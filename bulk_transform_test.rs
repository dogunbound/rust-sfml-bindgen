pub fn sfRenderWindow_clear(renderWindow: *mut sfRenderWindow, color: sfColor);
pub fn sfRenderWindow_setView(renderWindow: *mut sfRenderWindow, view: *const sfView);
pub fn sfRenderWindow_getView(renderWindow: *const sfRenderWindow) -> *const sfView;
pub fn sfRenderWindow_drawPrimitives(renderWindow: *mut sfRenderWindow, vertices: *const sfVertex, vertexCount: usize, type_: sfPrimitiveType, states: *const sfRenderStates);
pub fn sfWindow_setFramerateLimit(window: *mut sfWindow, limit: c_uint);
pub fn sfWindow_setJoystickThreshold(window: *mut sfWindow, threshold: f32);
