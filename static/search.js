const searchbar = document.getElementById('searchbar');
const items = Array.from(document.getElementsByClassName('item'));

searchbar?.addEventListener('input', e => {

    // Extract search query
    const query = e.target.value.toLowerCase();

    // If empty query -> show all items
    if(query == '') {
        for(let item of items) {
            item.hidden = false;
        }

    // If tag search
    } else if(query[0] == '#') {
        for(let item of items) {

            // Extract tag list
            let tags = item.dataset.tags;
            tags = tags.substring(1, tags.length - 1);

            // If item has searched tag -> show it; if not -> hide it
            if(tags.includes(query.substring(1))) {
                item.hidden = false;
            } else {
                item.hidden = true;
            }
        }

    // Normal search
    } else {
        for(let item of items) {

            // Extract title and description text
            const title       = item.firstElementChild?.innerHTML.toLowerCase();
            const description = item.firstElementChild?.nextElementSibling?.innerHTML.toLowerCase();

            // If title or description contains search query -> show it; else -> hide it
            if(title.includes(query) || description.includes(query)) {
                item.hidden = false;
            } else {
                item.hidden = true;
            }
        }
    }
})
