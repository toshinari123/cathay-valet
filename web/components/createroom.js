export default async function createRoom({ name, user_id, description }) {
    try {
        const url = "http://localhost:8080/rooms/create";
        console.log(JSON.stringify({ name: name, owner: user_id, description: description }));
        let result = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ name: name, owner: user_id, description: description })
        });
        console.log(result);
        return result.json();
    } catch (e) {
        console.log(e);
        return Promise.reject(e);
    }
}


