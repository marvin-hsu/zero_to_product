from flask import Flask, jsonify

app = Flask(__name__)


@app.route('/')
def mock_server_ready():
    return '', 200


@app.route('/v3/email/send', methods=['POST'])
def send():
    return jsonify({"message": "Subscribed successfully"}), 200


if __name__ == '__main__':
    app.run()
