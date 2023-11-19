document.addEventListener('DOMContentLoaded', function() {
    const li1 = document.getElementById('li1');
    const li2 = document.getElementById('li2');
    const li3 = document.getElementById('li3');
    const li4 = document.getElementById('li4');

    // Initially, li2 is not visible

    // When li3 is clicked, li4 becomes visible and li2 goes out
    li3.addEventListener('click', function() {
        li4.classList.add('show');
        li4.classList.remove('hide');
        li2.classList.add('hide');
        li2.classList.remove('show');
    });

    // When li1 is clicked, toggle it back
    li1.addEventListener('click', function() {
        li4.classList.add('hide');
        li4.classList.remove('show');
        li2.classList.add('show');
        li2.classList.remove('hide');
    });
});
