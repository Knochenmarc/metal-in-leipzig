(function (doc) {

    function storeFilters() {
        let formData = new FormData(doc.getElementById('event-filter'))
        let data = {};
        formData.forEach((value, key) => (data[key] = value));
        localStorage.setItem('filter', JSON.stringify(data));
    }

    let inputs = doc.querySelectorAll('input[form="event-filter"]')
    let init = {}
    let loaded = localStorage.getItem('filter')
    if (loaded) {
        init = JSON.parse(loaded)
    }

    inputs.forEach(function (input) {
        if (input.getAttribute('type') === 'checkbox' && input.getAttribute('name') in init) {
            input.checked = true
        }

        // need proper form change handler :-/
        input.addEventListener('change', storeFilters);
    });
})(document)