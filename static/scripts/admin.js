function delete_item(url, id) {
    form_data = new FormData();
    form_data.append("id", id);
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', url, true );
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      toast_success("Удалено!");
    }};
    link.send(form_data);
};


on('body', 'click', '#create_country', function() {
    let form = this.parentElement.parentElement.parentElement;
    if (!form.querySelector("#id_name").value) {
        form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
        return
      }
      form_data = new FormData(form);
    
      link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      link.open( 'POST', "/create_country/", true );
      link.onreadystatechange = function () {
      if ( link.readyState == 4 && link.status == 200 ) {
        location.reload()
      }};
      link.send(form_data);
});
on('body', 'click', '#edit_country', function() {
    _this = this;
    form = _this.parentElement.parentElement.parentElement;
    if (!form.querySelector("#id_name").value) {
        form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
        return
      }
      form_data = new FormData(form);
    
      link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      link.open( 'POST', "/edit_country/" + _this.getAttribute("data-pk") + "/", true );
      link.onreadystatechange = function () {
      if ( link.readyState == 4 && link.status == 200 ) {
        location.reload()
      }};
      link.send(form_data);
});

on('body', 'click', '.remove_country', function() {
    delete_item("/delete_country/", this.getAttribute("data-pk"));
    this.parentElement.remove();
});