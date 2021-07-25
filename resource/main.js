const ws = new WebSocket(`ws://${location.host}/ws`);

ws.addEventListener('open', (e) => console.log(e));

ws.addEventListener('message', (e) => console.log(e.data));

document.addEventListener('DOMContentLoaded',function(e){
    document.getElementById('sample').addEventListener('click', (e) => ws.send('ping'));
});
