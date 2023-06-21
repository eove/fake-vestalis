document.getElementById("server").value = location.hostname;

const connect = async () => {
    const target = document.getElementById("serial").value;
    const server = document.getElementById("server").value;
    const dateValue = document.getElementById("date").value;
    let now;
    if (dateValue) {
        now = new Date(dateValue).toISOString();
    } else {
        now = new Date().toISOString();
    } // should be done server side, done here for testing purpose only
    let signature_response = await fetch(`/sign/${target}/${now}`);
    let signature = await signature_response.json();
    document.getElementById("eo-connect").src = `https://${server}:8000/?target=${target}&timestamp=${now}&token=${signature}`
}
const disconnect = () => {
    document.getElementById("eo-connect").src = 'about:blank';
}
