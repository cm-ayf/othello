const ws = new WebSocket(`ws://${location.host}/ws`);


ws.addEventListener('message', (e) => {
    let json = JSON.parse(e.data);
    console.log(json);
    let board = document.getElementById('board');
    json.state.forEach((row) => {
        let rowElement = board.appendChild(document.createElement('tr'));
        row.forEach(sq => {
            let sqElement = rowElement.appendChild(document.createElement('td'));
            sqElement.classList.add(`t${sq}`);
            sqElement.appendChild(document.createElement('div')).classList.add(`d${sq}`);
        });
    })
});

document.addEventListener('DOMContentLoaded', (e) => {
    ws.addEventListener('open', (e) => ws.send('DOMContentLoaded'));
});