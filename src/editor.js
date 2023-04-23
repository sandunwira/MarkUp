const converter = new showdown.Converter();
const editor = CodeMirror.fromTextArea(document.getElementById('editor'), {
    mode: 'markdown',
    lineNumbers: true,
    theme: 'dracula',
    matchBrackets: true
});
const preview = document.getElementById('preview');
function updatePreview() {
    preview.innerHTML = converter.makeHtml(editor.getValue());
}
editor.on('change', updatePreview);
updatePreview();