(function () {
    var is_supported = false;
    
    //WebAssembly
    var webAssembly = typeof WebAssembly === "object" && typeof WebAssembly.instantiate === "function";

    //customElements
    var webComponents = typeof customElements === "object" && typeof customElements.define === "function";

    is_supported = webAssembly && webComponents;

    if (!is_supported) {
        document.getElementById("root").innerHTML = 
            "<div style='text-align:center;margin-top:150px'>" + 
                "Sorry, please use the latest version of Chrome" +
                "<br />" + 
                "抱歉，请使用最新版本的Chrome浏览器" +
            "</div>";
    }
})();