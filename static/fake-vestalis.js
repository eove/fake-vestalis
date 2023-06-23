let server;

(async () =>{
    console.log("requesting server")
    server = await (await fetch("/connect-name")).json();
    console.log(server)
})()

const connect = async () => {
    const target = document.getElementById("serial").value;
    const dateValue = document.getElementById("date").value;
    let now;
    if (dateValue) {
        now = new Date(dateValue).toISOString();
    } else {
        now = new Date().toISOString();
    } // should be done server side, done here for testing purpose only
    let signature_response = await fetch(`/sign/${target}/${now}`);
    let {token, uuid} = await signature_response.json();
    document.getElementById("eo-connect").src = `https://${server}/view/?target=${target}&timestamp=${now}&token=${token}&uuid=${uuid}`
}
const disconnect = () => {
    document.getElementById("eo-connect").src = 'about:blank';
}
