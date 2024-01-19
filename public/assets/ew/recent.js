function connect() {
}

function loaded() {
    console.log("loaded");
    var recent_id = document.getElementById("recent-id");

    var ws = new WebSocket(location.origin.replace("http", "ws") + "/api/v1/recent_levels/ws");
    ws.onopen = function(e) {
        recent_id.visible = true;
    }
    ws.onerror = function(e) {
        console.log("error", e);
        recent_id.visible = true;
        recent_id.innerHTML = "error";
    }
    ws.onmessage = function(e) {
        console.log("message", e);
        var data = JSON.parse(e.data);
        recent_id.innerHTML = data.level_id;
    };
    ws.onclose = function(e) {
        recent_id.visible = false;
        console.log("bye bye ws", e);
    };
}