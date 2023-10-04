let server;

(async () =>{
    console.log("requesting server")
    server = await (await fetch("/connect-name")).json();
    console.log(server)
})()

const connect = async (target) => {
    let signature_response = await fetch(`/sign/${target}`);
    let {signature, data, nonce} = await signature_response.json();
    document.getElementById("eo-connect").src = `https://${server}/view/?data=${data}&signature=${signature}&nonce=${nonce}`
}
const disconnect = () => {
    document.getElementById("eo-connect").src = 'about:blank';
}
