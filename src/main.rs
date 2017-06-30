extern crate xcb;

use xcb::Connection;

fn main() {
    let (connection, screen) = Connection::connect(None).unwrap();
    let window = connection.generate_id();

    let screen = connection.get_setup().roots().nth(screen as usize).unwrap();
    xcb::create_window(
        &connection,
        xcb::COPY_FROM_PARENT as u8,
        window, screen.root(),
        0, 0, 1, 1,
        0,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        &[(
            xcb::CW_EVENT_MASK,
            xcb::EVENT_MASK_STRUCTURE_NOTIFY | xcb::EVENT_MASK_PROPERTY_CHANGE
        )]
    );
    connection.flush();
}
