const ws = new WebSocket(`ws://${location.host}/ws`);

document.addEventListener('DOMContentLoaded', (e) => {
    ws.addEventListener('open', () => ws.send('reload'));
});

document.addEventListener('close', () => {
    ws.addEventListener('open', () => ws.send('close'));
});

ws.addEventListener('message', (e) => {
    let json = JSON.parse(e.data);
    console.log(json);
    let oldBoard = document.getElementById('board');
    let board = oldBoard.cloneNode(false);
    oldBoard.parentNode.replaceChild(board, oldBoard);
    json.state.forEach((row, i) => {
        let rowElement = board.appendChild(document.createElement('tr'));
        row.forEach((sq, j) => {
            let sqElement = rowElement.appendChild(document.createElement('td'));
            sqElement.classList.add(`t${sq}`);
            if (json.new && json.new[0] == i && json.new[1] == j) {
                sqElement.classList.add('new');
            };
            switch (sq) {
                case 3:
                case 4:
                    sqElement.addEventListener('click', (e) => {
                        ws.send(`put ${i} ${j}`);
                    });
                    break;
            };
            sqElement.appendChild(document.createElement('div')).classList.add(`d${sq}`);
        });
    });
});

