(() => {
    function updateGenreFields() {
        let toDiscard = [];
        let lastElement = null;
        for (let element of document.querySelectorAll('#genre').values()) {
            if (!element.value) {
                toDiscard.push(element)
            }
            lastElement = element;
        }
        while (toDiscard.length > 1) {
            toDiscard.pop().remove()
        }
        if (toDiscard.length === 0) {
            let element = document.createElement('input')
            element.type = "text"
            element.id = "genre"
            element.onchange = doUpdate
            lastElement.insertAdjacentElement('afterend', element)
        }
    }

    function getGenreData() {
        let genre = [];
        for (let element of document.querySelectorAll('#genre').values()) {
            let value = element.value;
            // starts and ends with character, only has lowercase characters and dashes inbetween
            if (/^[a-z][a-z-]*[a-z]$/.test(value)) {
                genre.push(value)
            } else {
                console.log("Invalid genre")
            }
        }
        let options = {};
        let useOptions = false;
        let location = document.querySelector('#location').value;
        if (location && location !== '0') {
            options.location = location;
            useOptions = true;
        }
        let category = document.querySelector('input[name="category"]:checked').value;
        if (category && category !== '0') {
            options.category = category;
            useOptions = true;
        }
        let base_url = window.location.href;
        if (base_url.endsWith('.html') || base_url.endsWith('/')) {
            base_url = base_url.substring(0, base_url.lastIndexOf('/'));
        }
        let genres = genre.join('+')
        let params = "";
        if (useOptions) {
            params = '?' + new URLSearchParams(options);
        }
        let url = `${base_url}/genre/${genres}${params}`
        document.querySelector('#result').innerHTML = url
    }

    function doUpdate() {
        updateGenreFields()
        getGenreData()
    }

    addEventListener("load", () => {
        for (let element of document.querySelector('#genre_feed').children) {
            element.onchange = doUpdate
        }
        doUpdate()
    })
})();