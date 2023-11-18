export default async function createConversation({ user_id, room_id, message }) {
    try {
        const url = "http://localhost:8080/conversations/create";
        console.log("hello");
        let result = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ user_id, room_id, message })
        });
        console.log(result);
        return result.json();
    } catch (e) {
        console.log(e);
        return Promise.reject(e);
    }
}


