function delete_item(url, id) {
    form_data = new FormData();
    form_data.append("id", id);
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', url, true )
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      toast_info("Сделано!");
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

    form.querySelector("#create_country").setAttribute("disabled", "true");
    form.querySelector("#create_country").innerHTML = "Идет сохранение";

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

    form.querySelector("#edit_country").setAttribute("disabled", "true");
    form.querySelector("#edit_country").innerHTML = "Идет сохранение";

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

  form.querySelector("#create_region").setAttribute("disabled", "true");
  form.querySelector("#create_region").innerHTML = "Идет сохранение";

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

  form.querySelector("#edit_region").setAttribute("disabled", "true");
  form.querySelector("#edit_region").innerHTML = "Идет сохранение";

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

  form.querySelector("#create_city").setAttribute("disabled", "true");
  form.querySelector("#create_city").innerHTML = "Идет сохранение";

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

  form.querySelector("#edit_city").setAttribute("disabled", "true");
  form.querySelector("#edit_city").innerHTML = "Идет сохранение";

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

  form.querySelector("#create_district").setAttribute("disabled", "true");
  form.querySelector("#create_district").innerHTML = "Идет сохранение";

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

  form.querySelector("#edit_district").setAttribute("disabled", "true");
  form.querySelector("#edit_district").innerHTML = "Идет сохранение";
  
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

  form.querySelector("#create_place").setAttribute("disabled", "true");
  form.querySelector("#create_place").innerHTML = "Идет сохранение";

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

  form.querySelector("#edit_place").setAttribute("disabled", "true");
  form.querySelector("#edit_place").innerHTML = "Идет сохранение";

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

  form.querySelector("#create_deceased").setAttribute("disabled", "true");
  form.querySelector("#create_deceased").innerHTML = "Идет сохранение";

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_deceased/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});


function get_image_value(_input, _path) {
  entrou = false;
  _input.click();
  _input.onchange = function() {
      if (!entrou) {
          _input.value = _path;
          reader = new FileReader();
          reader.readAsDataURL(_input.files[0])
      }
      entrou = true;
      setTimeout(function() {
          entrou = false
      }, 1000)
  }
};

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
  form.querySelector("#edit_deceased").setAttribute("disabled", "true");
  form.querySelector("#edit_deceased").innerHTML = "Идет сохранение";

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
  delete_item("/delete_deceased/", this.getAttribute("data-pk"));
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

on('body', 'click', '.remove_admin', function() {
  _this = this;
  delete_item("/users/remove_staff/", _this.getAttribute("data-pk"));
  _this.classList.remove("remove_admin");
  _this.classList.add("create_admin");
  _this.innerHTML = "Сделать админом";
  console.log("click");
});
on('body', 'click', '.toggle_admin', function() {
  _this = this;
  if (_this.classList.contains("admin")) {
    delete_item("/users/remove_staff/", _this.getAttribute("data-pk"));
    _this.classList.remove("admin");
    _this.innerHTML = "Сделать админом";
  }else {
    delete_item("/users/create_admin/", _this.getAttribute("data-pk"));
    _this.classList.add("admin");
    _this.innerHTML = "Убрать из админов";
  }
  console.log("click");
});