const express = require('express')
const user = require('./models/user')
const cors = require('cors')
const app = express();
const enc = new TextEncoder();
const {ethers} = require('ethers');

function strToBin(str) {
    return Uint8Array.from(atob(str), c => c.charCodeAt(0));
}

app.use(express.json());
app.use(cors());




app.post('/login', async (req, res) => {

    const {Name, AccountID, UpiID, RawID}=req.body;

    const RoundUpContractAddress=await deploy();

    console.log(Name);
    const usercreated = new user({Name, AccountID, UpiID, RawID,RoundUpContractAddress});
    console.log(req.body);
    let result = await usercreated.save();
    res.send(result);
})


app.post('/createpublickey', async (req, res) => {

    const publicKey = {
        // random, cryptographically secure, at least 16 bytes
        challenge: enc.encode('someRandomStringThatSHouldBeReLLYLoooong&Random'),
        // relying party
        rp: {
            id:'localhost',
            name: 'closedcred'
        },
        user: {
            id: enc.encode(req.body.address),
            name: req.body.name,
            displayName: req.body.name
        },
        authenticatorSelection: {
            userVerification: "preferred"
        },
        attestation: 'direct',
        pubKeyCredParams: [
            {
                type: "public-key", alg: -7 // "ES256" IANA COSE Algorithms registry
            }
        ]
    }

    console.log("Response send");
    res.send(publicKey);

})



app.listen(5000)