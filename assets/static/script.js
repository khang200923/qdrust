var chess = new Chess();

const delay = ms => new Promise(res => setTimeout(res, ms));

async function render() {
    let boardNode = document.getElementById("chess-board");
    boardNode.innerHTML = "";
    let tableBody = document.createElement("tbody");
    for (let i = 7; i >= 0; i--) {
        tableRow = document.createElement("tr");
        for (let j = 0; j < 8; j++) {
            tableCell = document.createElement("td");
            tableCell.id = `${j},${i}`;
            if (chess.blocks[tableCell.id]) {tableCell.innerText = "❌";};
            if (chess.wPos[0] == j && chess.wPos[1] == i) {tableCell.innerText = "♕";};
            if (chess.bPos[0] == j && chess.bPos[1] == i) {tableCell.innerText = "♛";};
            if (chess.end() != null) {
                if (chess.end() && chess.wPos[0] == j && chess.wPos[1] == i) {tableCell.innerText = "♕";}
                if ((!chess.end()) && chess.bPos[0] == j && chess.bPos[1] == i) {tableCell.innerText = "♛";};
            }
            if (chess.possibles().map((x) => `${x[0]},${x[1]}`).includes(tableCell.id)) {
                tableCell.classList.add("mov");
                tableCell.onclick = async function() {
                    if (chess.end() != null) {return;}
                    chess.move([j, i]);
                    await render();
                    if (chess.end() != null) {return;}
                    await delay(50);
                    let botMove = await useBot(chess);
                    chess.move(botMove);
                    await render();
                }
            };
            if ((i+j) % 2 == 1) {tableCell.classList.add("light");}
            else {tableCell.classList.add("dark");}
            tableRow.appendChild(tableCell);
        }
        tableBody.appendChild(tableRow);
    }
    boardNode.appendChild(tableBody);
    if (chess.end() === true)
    {
        let msg = document.createElement("span");
        msg.setAttribute("id", "msg");
        msg.innerHTML = "<p style='color: white;'>White wins! <a href='' style='color: white;'>Retry?</a></p>";
        document.getElementById("container").appendChild(msg);
    }
    if (chess.end() === false)
    {
        let msg = document.createElement("span");
        msg.setAttribute("id", "msg");
        msg.innerHTML = "<p style='color: white;'>Black wins! <a href='' style='color: white;'>Retry?</a></p>";
        document.getElementById("container").appendChild(msg);
    }
}

window.onload = function() {
    render()
}