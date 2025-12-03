pub fn styles() -> String {
    r#"
    <style>
    body {
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        background: linear-gradient(to right, #a8dadc, #457b9d);
        margin: 0;
        padding: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 100vh;
        animation: gradientAnimation 10s ease infinite;
        background-size: 200% 200%;
    }

    @keyframes gradientAnimation {
        0% {
            background-position: 0% 50%;
        }
        50% {
            background-position: 100% 50%;
        }
        100% {
            background-position: 0% 50%;
        }
    }

    .container {
        background-color: rgba(255, 166, 0, 0.9);
        border-radius: 12px;
        box-shadow: 0 6px 12px rgba(0, 0, 0, 0.3);
        padding: 30px;
        width: 90%;
        max-width: 700px;
        transition: transform 0.3s ease-in-out;
    }

    .container:hover {
        transform: scale(1.03);
    }

    h1 {
        color: #1d3557;
        text-align: center;
        word-break: break-word;
        margin-bottom: 20px;
        text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
        font-size: 2.5em;
        letter-spacing: 1px;
        font-family: 'Comic Sans MS', cursive, sans-serif; /* Font Comic Sans MS */
    }

    form {
        display: flex;
        flex-direction: column;
        margin-bottom: 20px;
    }

    label {
        margin-bottom: 8px;
        font-weight: bold;
        color: #1d3557;
        text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
        font-family: 'Comic Sans MS', cursive, sans-serif; /* Font Comic Sans MS */
    }

    input[type="text"] {
        padding: 12px;
        margin-bottom: 15px;
        border-radius: 8px;
        border: none;
        box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
        font-size: 16px;
        transition: box-shadow 0.3s ease;
        background-color: #e7cd75ff;
        font-family: 'Comic Sans MS', cursive, sans-serif; /* Font Comic Sans MS */
        color: #457b9d; /* Biru terang */
        text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1); /* Efek teks 3D */
    }

    input[type="text"]:focus {
        box-shadow: inset 0 3px 6px rgba(0, 0, 0, 0.2);
        outline: none;
    }

    button {
        background-color: #457b9d;
        color: white;
        border: none;
        cursor: pointer;
        flex: 1;
        margin: 5px;
        padding: 12px 24px;
        border-radius: 8px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.2);
        transition: background-color 0.3s ease, transform 0.3s ease;
        font-family: 'Comic Sans MS', cursive, sans-serif; /* Font Comic Sans MS */
    }

    button:hover {
        background-color: #1d3557;
        transform: translateY(-2px);
        box-shadow: 0 6px 8px rgba(0, 0, 0, 0.3);
    }

    ul {
        list-style: none;
        padding: 0;
    }

    li {
        padding: 15px;
        border-bottom: 1px solid #a8dadc;
        display: flex;
        justify-content: space-between;
        align-items: center;
        word-break: break-word;
        transition: background-color 0.3s ease, transform 0.3s ease;
        font-family: 'Comic Sans MS', cursive, sans-serif; /* Font Comic Sans MS */
        color: #457b9d; /* Biru terang */
        text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1); /* Efek teks 3D */
    }

    li:last-child {
        border-bottom: none;
    }

    li:hover {
        background-color: #8afa8aff;
        transform: translateY(-1px);
    }

    .completed {
        text-decoration: line-through;
        color: #778da9;
    }

    .actions {
        display: flex;
        gap: 8px;
    }

    .actions button {
        margin-left: 0;
        padding: 8px 16px;
        font-size: 14px;
        line-height: 1;
        border: none;
        border-radius: 6px;
        cursor: pointer;
        transition: background-color 0.3s ease, transform 0.3s ease;
        font-family: 'Comic Sans MS', cursive, sans-serif; /* Font Comic Sans MS */
    }

    .actions button:hover {
        transform: translateY(-1px);
    }

    .actions button.delete {
        background-color: #e63946;
        color: white;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    }

    .actions button.delete:hover {
        background-color: #c62a35;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
    }

    .actions button.complete-button {
        background-color: #2a9d8f;
        color: white;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    }

    .actions button.complete-button:hover {
        background-color: #228075;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
    }

    .edit-form {
        display: none;
        margin-top: 10px;
    }

    .editing .edit-form {
        display: flex;
        flex-direction: column;
    }

    .editing span {
        display: none;
    }

    .modal {
        display: none;
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        z-index: 1000;
    }

    .modal-content {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background-color: white;
        padding: 30px;
        border-radius: 12px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
        text-align: center;
    }

    .modal-buttons {
        margin-top: 20px;
        display: flex;
        justify-content: center;
        gap: 15px;
    }

    .modal-buttons button {
        padding: 10px 20px;
        border: none;
        border-radius: 8px;
        cursor: pointer;
        font-size: 16px;
        transition: background-color 0.3s ease, transform 0.3s ease;
        font-family: 'Comic Sans MS', cursive, sans-serif; /* Font Comic Sans MS */
    }

    .modal-buttons button:hover {
        transform: translateY(-1px);
    }

    .modal-buttons button.delete {
        background-color: #e63946;
        color: white;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    }

    .modal-buttons button.delete:hover {
        background-color: #c62a35;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
    }

    .modal-buttons button.cancel {
        background-color: #457b9d;
        color: white;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    }

    .modal-buttons button.cancel:hover {
        background-color: #1d3557;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
    }

    @media (max-width: 600px) {
        .container {
            width: 95%;
            padding: 20px;
        }

        h1 {
            font-size: 28px;
        }

        input[type="text"] {
            padding: 10px;
            font-size: 14px;
        }

        button {
            padding: 10px 20px;
            font-size: 14px;
        }

        .actions button {
            padding: 6px 12px;
            font-size: 12px;
        }
    }
</style>
    "#.to_string()
}
