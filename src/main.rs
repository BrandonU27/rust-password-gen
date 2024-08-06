mod password_logic;

use password_logic::PasswordGenerator;
use slint::Weak;

slint::slint! {
    import { Button, VerticalBox, LineEdit, CheckBox } from "std-widgets.slint";

    export component App inherits Window {
        in property <string> password: "";
        VerticalBox {
            alignment: start;
            Text {
                text: "Password Generator";
                font-size: 24px;
                horizontal-alignment: center;
            }
            Text {
                text: password;
                font-size: 24px;
                height: 40px;
                vertical-alignment: center;
                horizontal-alignment: center;
            }
            VerticalLayout {
                alignment: center;
                LineEdit {
                placeholder-text: "Password length";
                }
                CheckBox {
                    text: "Include uppercase letters";
                    checked: true;
                }
                CheckBox {
                    text: "Include lowercase letters";
                    checked: true;
                }
                CheckBox {
                    text: "Include numbers";
                    checked: true;
                }
                CheckBox {
                    text: "Include symbols";
                    checked: true;
                }
            }
            HorizontalLayout {
                alignment: center;
                Button {
                    text: "Generate password";
                    clicked => {  }
                }
            }
        }
    }
}

fn main()
{
    let app: App = App::new().unwrap();
    let weak = app.as_weak();
    app.run().unwrap();
}
