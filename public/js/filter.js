(function (doc) {
    const storeFilters = () => {
        let formData = new FormData(doc.getElementById('event-filter'));
        let data = {};
        formData.forEach((value, key) => (data[key] = value));
        localStorage.setItem('filter', JSON.stringify(data));
    };
    const init = JSON.parse(localStorage.getItem('filter') || '{}');

    doc.querySelectorAll('input[form="event-filter"]').forEach(input => {
        if (input.getAttribute('type') === 'checkbox' && input.getAttribute('name') in init) {
            input.checked = true
        }

        /* need proper form change handler :-/ */
        input.addEventListener('change', storeFilters);
    });
})(document)