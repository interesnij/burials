function delete_item(url, id) {
    form_data = new FormData();
    form_data.append("id", id);
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', url, true )
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
        toast_error("Укажите название страны");
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
        toast_error("Укажите название страны");
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

on('body', 'click', '.publish_org', function() {
  delete_item("/organization/publish/", this.getAttribute("data-pk"));
  this.parentElement.innerHTML = "Одобрено";
});


on('body', 'click', '#create_region', function() {
  let form = this.parentElement.parentElement.parentElement;
  
  form.querySelector("#id_name").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_country").style.setProperty('border', 'unset', 'important');

  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название");
      return
  }
  else if (!form.querySelector("#id_country").value) {
    form.querySelector("#id_country").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите страну");
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

  form.querySelector("#id_name").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_country").style.setProperty('border', 'unset', 'important');

  if (!form.querySelector("#id_name").value) {
    form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите название");
    return
  }
  else if (!form.querySelector("#id_country").value) {
    form.querySelector("#id_country").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите страну");
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
  
  form.querySelector("#id_name").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_country").style.setProperty('border', 'unset', 'important');

  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название");
      return
  }
  else if (!form.querySelector("#id_country").value) {
    form.querySelector("#id_country").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите страну");
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
  
  form.querySelector("#id_name").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_country").style.setProperty('border', 'unset', 'important');

  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название");
      return
  }
  else if (!form.querySelector("#id_country").value) {
    form.querySelector("#id_country").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите страну");
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
  
  form.querySelector("#id_name").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_country").style.setProperty('border', 'unset', 'important');

  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название");
      return
  }
  else if (!form.querySelector("#id_country").value) {
    form.querySelector("#id_country").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите страну");
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
  
  form.querySelector("#id_name").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_country").style.setProperty('border', 'unset', 'important');

  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название");
      return
  }
  else if (!form.querySelector("#id_country").value) {
    form.querySelector("#id_country").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите страну");
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


on('body', 'click', '#create_place', function() {
  let form = this.parentElement.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_title").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_country").style.setProperty('border', 'unset', 'important');

  if (!form.querySelector("#id_title").value) {
      form.querySelector("#id_title").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название кладбища");
      return
  }
  else if (!form.querySelector("#id_country").value) {
    form.querySelector("#id_country").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите страну кладбища");
    return
  }
    form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_place/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});
on('body', 'click', '#edit_place', function() {
  _this = this;
  form = _this.parentElement.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_title").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_country").style.setProperty('border', 'unset', 'important');

  if (!form.querySelector("#id_title").value) {
      form.querySelector("#id_title").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название кладбища");
      return
  }
  else if (!form.querySelector("#id_country").value) {
    form.querySelector("#id_country").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите страну кладбища");
    return
  }

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/edit_place/" + _this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});

on('body', 'click', '.remove_place', function() {
  delete_item("/delete_place/", this.getAttribute("data-pk"));
  this.parentElement.remove();
});


on('body', 'click', '#create_deceased', function() { 
  let form = this.parentElement.parentElement.parentElement;
  form.querySelector("#id_first_name").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_last_name").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_birth_date").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_death_date").style.setProperty('border', 'unset', 'important');
  form.querySelector("#place_id").style.setProperty('border', 'unset', 'important');
  form.querySelector("#get_my_lat").style.setProperty('border', 'unset', 'important');
  form.querySelector("#get_my_lon").style.setProperty('border', 'unset', 'important');

  if (!form.querySelector("#id_first_name").value) {
      form.querySelector("#id_first_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите имя усопшего");
      return
  }
  else if (!form.querySelector("#id_last_name").value) {
    form.querySelector("#id_last_name").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите фамилию усопшего");
    return
  }
  else if (!form.querySelector("#id_birth_date").value) {
    form.querySelector("#id_birth_date").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите дату рождения усопшего");
    return
  }
  else if (!form.querySelector("#id_death_date").value) {
    form.querySelector("#id_death_date").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите дату смерти усопшего");
    return
  }
  else if (!form.querySelector("#place_id").value) {
    form.querySelector("#place_id").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите место захоронения усопшего");
    return
  }
  else if (!form.querySelector("#get_my_lat").value) {
    form.querySelector("#get_my_lat").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите широту местоположения усопшего");
    return
  }
  else if (!form.querySelector("#get_my_lon").value) {
    form.querySelector("#get_my_lon").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите долготу местоположения усопшего");
    return
  }

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_deceased/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});
on('body', 'click', '#edit_deceased', function() {
  _this = this; 
  form = _this.parentElement.parentElement.parentElement;
  form.querySelector("#id_first_name").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_last_name").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_birth_date").style.setProperty('border', 'unset', 'important');
  form.querySelector("#id_death_date").style.setProperty('border', 'unset', 'important');
  form.querySelector("#place_id").style.setProperty('border', 'unset', 'important');
  form.querySelector("#get_my_lat").style.setProperty('border', 'unset', 'important');
  form.querySelector("#get_my_lon").style.setProperty('border', 'unset', 'important');

  if (!form.querySelector("#id_first_name").value) {
      form.querySelector("#id_first_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите имя усопшего");
      return
  }
  else if (!form.querySelector("#id_last_name").value) {
    form.querySelector("#id_last_name").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите фамилию усопшего");
    return
  }
  else if (!form.querySelector("#id_birth_date").value) {
    form.querySelector("#id_birth_date").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите дату рождения усопшего");
    return
  }
  else if (!form.querySelector("#id_death_date").value) {
    form.querySelector("#id_death_date").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите дату смерти усопшего");
    return
  }
  else if (!form.querySelector("#place_id").value) {
    form.querySelector("#place_id").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите место захоронения усопшего");
    return
  }
  else if (!form.querySelector("#get_my_lat").value) {
    form.querySelector("#get_my_lat").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите широту местоположения усопшего");
    return
  }
  else if (!form.querySelector("#get_my_lon").value) {
    form.querySelector("#get_my_lon").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите долготу местоположения усопшего");
    return
  }
  
  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/edit_deceased/" + _this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});

on('body', 'click', '.remove_deceased', function() {
  delete_item("/delete_deceased/" + _this.getAttribute("data-pk") + "/", this.getAttribute("data-pk"));
  this.parentElement.remove();
});


on('body', 'click', '.publish_place', function() {
  delete_item("/place/publish/", this.getAttribute("data-pk"));
  this.parentElement.remove();
});
on('body', 'click', '.unpublish_place', function() {
  delete_item("/place/unpublish/", this.getAttribute("data-pk"));
  this.parentElement.remove();
});

on('body', 'click', '.publish_organization', function() {
  delete_item("/organization/publish/", this.getAttribute("data-pk"));
  this.parentElement.remove();
});
on('body', 'click', '.unpublish_organization', function() {
  delete_item("/organization/unpublish/", this.getAttribute("data-pk"));
  this.parentElement.remove();
});

on('body', 'click', '.remove_staff', function() {
  this.classList.remove("remove_staff");
  this.classList.add("create_admin");
  this.innerHTML = "Сделать админом";
  delete_item("/users/remove_staff/", this.getAttribute("data-pk"));
});
on('body', 'click', '.create_admin', function() {
  this.classList.remove("create_admin");
  this.classList.add("remove_staff");
  this.innerHTML = "Убрать из админов";
  delete_item("/users/create_admin/", this.getAttribute("data-pk"));
});