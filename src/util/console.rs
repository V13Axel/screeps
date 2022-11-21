use js_sys::JsString;
use web_sys::console;

// What a crazy hack.
pub fn clear_console() {
    console::log_1(
        &JsString::from(
            "<script>angular.element(document.getElementsByClassName('fa fa-trash ng-scope')[0].parentNode).scope().Console.clear()</script>"
        )
    );
}

