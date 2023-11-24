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


on('body', 'click', '#create_region', function() {
  let form = this.parentElement.parentElement.parentElement;
  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      return
    }
    form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_region/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});
on('body', 'click', '#edit_region', function() {
  _this = this;
  form = _this.parentElement.parentElement.parentElement;
  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      return
    }
    form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/edit_region/" + _this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});

on('body', 'click', '.remove_region', function() {
  delete_item("/delete_region/", this.getAttribute("data-pk"));
  this.parentElement.remove();
});


on('body', 'click', '#create_city', function() {
  let form = this.parentElement.parentElement.parentElement;
  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      return
    }
    form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_city/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});
on('body', 'click', '#edit_city', function() {
  _this = this;
  form = _this.parentElement.parentElement.parentElement;
  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      return
    }
    form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/edit_city/" + _this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});

on('body', 'click', '.remove_city', function() {
  delete_item("/delete_city/", this.getAttribute("data-pk"));
  this.parentElement.remove();
});

on('body', 'click', '#create_district', function() {
  let form = this.parentElement.parentElement.parentElement;
  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      return
    }
    form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_district/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});
on('body', 'click', '#edit_district', function() {
  _this = this;
  form = _this.parentElement.parentElement.parentElement;
  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      return
    }
    form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/edit_district/" + _this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});

on('body', 'click', '.remove_district', function() {
  delete_item("/delete_district/", this.getAttribute("data-pk"));
  this.parentElement.remove();
});