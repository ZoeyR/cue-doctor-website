//url: /products
//returns json that is array of products

$(document).ready(function() {

// the code is inside the ready function - 
// makes sure that DOM is loaded before using jQuery

if(!sessionStorage.order && !sessionStorage.count)
{
    sessionStorage.count = 0;
}

$("#cartcount").text(sessionStorage.count);

  //alert("Now going to make POST ajax call - results will appear soon");
$.get("/products", function(data,status) {
        //obj = JSON.parse(data);
        for(i = 0; i < data.length; i++) {
            if(data[i].name == document.title) {
                $("#bodytext").text(obj[i].description);
            }

        }
        
});






$("#order").click(function() {
    $.get("/products", function(data,status) {
        //obj = JSON.parse(data);
        for(i = 0; i < data.length; i++) {
            if(data[i].name == document.title) {
                //$("#bodytext").text(obj[i].description);
                sessionStorage.setItem("order", JSON.stringify(data[i]));
                sessionStorage.count += 1;
            }
        }
    });
})
//$("#tester").text("Testing is working");


}); // end of document ready function


