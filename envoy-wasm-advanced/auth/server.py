
from flask import Flask, request, Response

app = Flask(__name__)

@app.get("/check")
def check():
    token = request.headers.get("authorization", "")
    if token.strip() == "Bearer allow":
        return Response("ok\n", status=200)
    return Response("no\n", status=403)

app.run(host="0.0.0.0", port=8080)
