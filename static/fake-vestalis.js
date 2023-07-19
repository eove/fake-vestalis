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
    let {signature, data, nonce} = await signature_response.json();
    document.getElementById("eo-connect").src = `https://${server}/view/?data=${data}&signature=${signature}&nonce=${nonce}`
}
const disconnect = () => {
    document.getElementById("eo-connect").src = 'about:blank';
}
