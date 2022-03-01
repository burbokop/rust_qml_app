use cstr::cstr;
use qmetaobject::prelude::*;
use std::env;

// The `QObject` custom derive macro allows to expose a class to Qt and QML
#[derive(QObject, Default)]
struct Greeter {
    // Specify the base class with the qt_base_class macro
    base: qt_base_class!(trait QObject),
    // Declare `name` as a property usable from Qt
    name: qt_property!(QString; NOTIFY name_changed),
    // Declare a signal
    name_changed: qt_signal!(),
    // And even a slot
    compute_greetings: qt_method!(fn compute_greetings(&self, verb: String) -> QString {
        format!("{} {}", verb, self.name.to_string()).into()
    })
}

fn main() {
    println!("ssssStart.....");
    env::set_var("QT_QPA_PLATFORM", "pocketbook2");

    qml_register_type::<Greeter>(cstr!("Greeter"), 1, 0, cstr!("Greeter"));

    let mut engine = QmlEngine::new();

    engine.load_data(r#"
        import QtQuick 2.6
        import QtQuick.Window 2.0
        // Import our Rust classes
        import Greeter 1.0
        import com.pocketbook.controls 1.0


        Window {
            width: DeviceInfoProvider.screenWidth
            height: DeviceInfoProvider.screenHeight - DeviceInfoProvider.panelHeight
            visible: true
            // Instantiate the rust struct
            Greeter {
                id: greeter;
                // Set a property
                name: "World"
            }
            Text {
                anchors.centerIn: parent
                // Call a method
                text: greeter.compute_greetings("hello")
            }
        }
    "#.into());
    engine.exec();
}