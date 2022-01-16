const homePage = document.location.origin;

function forkClicked () {
    const paste = document.getElementById('pasteContent');
    let text = paste.innerText;
    localStorage["forkText"] = text;

    console.log(text);

    window.location = homePage;
}

function newPasteClicked () {
    window.location = homePage;
}