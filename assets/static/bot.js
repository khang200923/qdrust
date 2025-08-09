function blocks_to_u64(blocks) {
    let u64 = BigInt(0);
    for (let i = 0; i < 64; i++) {
        if (blocks[`${i % 8},${Math.floor(i / 8)}`]) {
            u64 |= BigInt(1) << BigInt(i);
        }
    }
    return u64;
}

async function useBot(chess) {
    let wqueen = chess.wPos[0] + chess.wPos[1]*8;
    let bqueen = chess.bPos[0] + chess.bPos[1]*8;
    let blocks = blocks_to_u64(chess.blocks);
    let response = await fetch("/bot", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${token}`
        },
        body: JSON.stringify({
            "state_repr": {
                "wqueen": wqueen,
                "bqueen": bqueen,
                "blocks": blocks.toString(),
                "is_white_turn": chess.wFirst
            }
        })
    });
    if (!response.ok) {
        throw new Error("Failed to get bot move");
    }
    let data = await response.json();
    return [data.move_made % 8, Math.floor(data.move_made / 8)];
}