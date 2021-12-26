const body = document.querySelector('body');
const form = document.querySelector('form');
const grid_form = document.querySelector('.grid_form');
const upload_card = document.querySelector('#upload_card');
const textarea = document.querySelector('textarea');
const select = document.querySelector('select');
const submitButton = document.querySelector('button[type="submit"]');

const onInput = () => {
    submitButton.classList.toggle('hidden', !textarea.value);
    select.classList.toggle('hidden', !textarea.value);
}
textarea.addEventListener('input', onInput);
onInput();

document.body.addEventListener('keydown', (e) => {
    if (e.key === 'Enter' && e.ctrlKey) {
        form.submit();
    }
});

async function postData(url = '', data) {
    const response = await fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: data
    });

    const text = await response.text();
    return text;
}

function preventDefaults(e) {
    e.preventDefault()
    e.stopPropagation()
}

// Prevent default drag behaviors
['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
    form.addEventListener(eventName, preventDefaults, false);
    document.body.addEventListener(eventName, preventDefaults, false);
});

// highlight the dragarea
['dragenter', 'dragover'].forEach(eventName => {
    form.addEventListener(eventName, highlight, false);
});

// unhighlight the dragarea
['dragleave'].forEach(eventName => {
    form.addEventListener(eventName, unhighlight, false);
});

function highlight(e) {
    form.classList.add('highlight');
    grid_form.classList.add('hidden');
}

function unhighlight(e) {
    form.classList.remove('highlight');
    grid_form.classList.remove('hidden');
}

// Files are dropped
function dropHandler(ev) {
    ev.preventDefault();

    // Give a visual cue
    upload_card.classList.add('show');
    grid_form.classList.add('hidden');

    if (ev.dataTransfer.items) {
        var item = ev.dataTransfer.items[0];
        var blob = item.getAsFile();
        const ext = blob.name.split(".")[1];
        var url = window.location.href;

        postData(url, blob)
            .then(data => {
                window.location.href = data + "." + ext;

                // remove the jazz for if user returns to the prev page
                upload_card.classList.remove('show');
                grid_form.classList.remove('hidden');
                form.classList.remove('highlight');
            })
            .catch(function (err) {
                console.info(err + " url: " + url);
            });
    }
}


// pasting files from the clipboard
document.onpaste = function (pasteEvent) {
    var item = pasteEvent.clipboardData.items[0];
    var blob = item.getAsFile();

    if (blob !== null && blob !== '') {
        var url = window.location.href;

        postData(url, blob)
            .then(data => {
                window.location.href = data;
            })
            .catch(function (err) {
                console.info(err + " url: " + url);
            });
    }
}