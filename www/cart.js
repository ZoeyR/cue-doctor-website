$(document).ready(function () {

    // the code is inside the ready function - 
    // makes sure that DOM is loaded before using jQuery

    if (!sessionStorage.order || !sessionStorage.count) {
        return;
    }
    let order = {};
    order.address = "none";
    order.items = [];
    let order_items = JSON.parse(sessionStorage.order);
    let total = 0;
    for (let item in order_items) {
        total += order_items[item].price;
        order.items.push({ product: order_items[item], quantity: 1 });
        $("#order_items").append('<div class="row"><div class="col-md-6">' + order_items[item].name + '</div><div class="col-md-6">$' + order_items[item].price / 100 + '</div></div>');
    }

    $("#order_cost").text("$" + total / 100);

    $("#submit_order").click(function () {
        $.ajax({
            'type': 'POST',
            'url': '/orders',
            'contentType': 'application/json',
            'data': JSON.stringify(order),
            'dataType': 'json',
            'success': function (data, status) {
                console.log(data);
            }
        });
    })
});


