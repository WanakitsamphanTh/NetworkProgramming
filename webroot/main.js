/*document.addEventListener("DOMContentLoaded", function() {
    const form = document.getElementById("post-form");
    const input = document.getElementById("post-input");
    const board = document.getElementById("board-list");

    form.addEventListener("submit", function(event) {
        event.preventDefault();

        const inputValue = input.value.trim();

        if (inputValue) {
            const current = board.getElementsByTagName("li");
            const newNumber = current.length + 1;

            const newLi = document.createElement("li");

            const newA = document.createElement("a");
            const newP1 = document.createElement("p");
            newP1.textContent = `>>${newNumber}`;
            newA.appendChild(newP1);

            const newP2 = document.createElement("p");
            newP2.textContent = inputValue;

            newLi.appendChild(newA);
            newLi.appendChild(newP2);

            board.appendChild(newLi);

            input.value = "";
        }
    });
});*/

var from, input, board;
var num_of_comments;

$(document).ready(function(){
    form = $("#post-form");
    input = $("#post-input");
    uname = $("#post-name");
    board = $("#board-list");

    form.on("submit",function(e){
        e.preventDefault();
        const inputValue = input.val().replaceAll("\n","<br>").replaceAll(",","<comma>");
        const nameValue = uname.val();
        console.log(inputValue)

        if(inputValue){
            const current = board.children("li");
            const date = formatDate(new Date());
            const data = createJSON(date,nameValue,inputValue)
            sendToServer(data);
            input.val("");
        }

    });

    displayAllMessages();
});

const formatDate = (date) => {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0'); // Months are 0-indexed
    const day = String(date.getDate()).padStart(2, '0');

    const hour = String(date.getHours());
    const minute = String(date.getMinutes()).padStart(2,'0');

    return `${year}-${month}-${day} ${hour}:${minute}`;
};

//コメントを１つ表示
function displayComment(data){
    const newLi = $("<li></li>");
    const newA = $("<a></a>");
    const newH = $("<h4></h4>");

    newH.text(`${data.name}さんより（${data.date}）`);
    newA.append(newH);
    
    const newP = $("<div></div>");

    const lines = data.content.replaceAll("<comma>",",").split("<br>");
    let content = ""
    lines.forEach((val) => {
        content = content + "<p>" + val + "</p>";
    });
    newP.append(content);

    newLi.append(newA);
    newLi.append(newP);

    board.append(newLi);
}

//コメントをJSON形式にする
function createJSON(date,name,content){
    return {
        "date": date,
        'name': name,
        "content": content
    };
}

//コメントをサーバに送信し、画面に表示する
function sendToServer(data) {
    const url = '/send';
    $.post(url,JSON.stringify(data), function(res){
        res = JSON.parse(res);
        displayComment(res);
        num_of_comments += 1;
    });
}

//最初にサーバから全部のコメントを読み込む
function displayAllMessages(){
    const url = '/getall';
    $.post(url,"").done(function(res){
        let list = JSON.parse(res);
        num_of_comments = list.length;
        list.forEach((msg) => displayComment(msg))
    });
}

setInterval(function(){
    const url = '/getall';
    $.post(url,"").done(function(res){
        let list = JSON.parse(res);
        while(num_of_comments < list.length){
            displayComment(list[num_of_comments]);
            num_of_comments += 1;
        }
    });
},5000);