export default async function joinRoom({ room_id, user_id }) {
    try {
        const url = "http://localhost:8080/rooms/join";
        console.log("hello (joinroom)");
        let result = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ room_id, user_id })
        });
        console.log(result);
        return result.json();
    } catch (e) {
        console.log(e);
        return Promise.reject(e);
    }
}


