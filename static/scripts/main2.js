function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};

function getCookie(name) {
  const cookies = document.cookie.split(';');
  for (let i = 0; i < cookies.length; i++) {
      let c = cookies[i].trim().split('=');
      if (c[0] === name) {
          return c[1];
      }
  }
  return "";
}
function setCookie(name, value, days) {
  let cookie = `${name}=${encodeURIComponent(value)}`;
  if (days) {
      const expiry = new Date();
      expiry.setDate(expiry.getDate() + days);
      cookie += `; expires=${expiry.toUTCString()}`;
  }
  document.cookie = cookie + "; path=/";
};
function eraseCookie(name) {   
  document.cookie = name+'=; Max-Age=-99999999;';  
}
function deleteAllCookies() {
  const cookies = document.cookie.split(";");
  setCookie("userrr", 0, 90);
  for (let i = 0; i < cookies.length; i++) {
      const cookie = cookies[i];
      const eqPos = cookie.indexOf("=");
      const name = eqPos > -1 ? cookie.substr(0, eqPos) : cookie;
      document.cookie = name + "=;expires=Thu, 01 Jan 1970 00:00:00 GMT";
  }
}
on('body', 'click', '.logout_hundler', function() {
  window.location.href = "/";
  setCookie("userrr", 0, 90);
  eraseCookie("userrr");
  window.location.href = "/";
  eraseCookie("userrr");
  window.location.href = "/";
})

class ToastManager {
    constructor() {
        this.id = 0;
        this.toasts = [];
        this.icons = {
            'SUCCESS': "",
            'ERROR': '',
            'INFO': '',
            'WARNING': '',
        };
        var body = document.querySelector('body');
        this.toastsContainer = document.createElement('div');
        this.toastsContainer.classList.add('toasts', 'border-0');
        body.appendChild(this.toastsContainer)
    }
    showSuccess(message) {
        return this._showToast(message, 'SUCCESS')
    }
    showError(message) {
        return this._showToast(message, 'ERROR')
    }
    showInfo(message) {
        return this._showToast(message, 'INFO')
    }
    showWarning(message) {
        return this._showToast(message, 'WARNING')
    }
    _showToast(message, toastType) {
        var newId = this.id + 1;
        var newToast = document.createElement('div');
        newToast.style.display = 'inline-block';
        newToast.classList.add(toastType.toLowerCase());
        newToast.classList.add('toast');
        newToast.innerHTML = `<progress max="100"value="0"></progress><h3>${message}</h3>`;
        var newToastObject = {
            id: newId,
            message,
            type: toastType,
            timeout: 4000,
            progressElement: newToast.querySelector('progress'),
            counter: 0,
            timer: setInterval(() => {
                newToastObject.counter += 1000 / newToastObject.timeout;
                newToastObject.progressElement.value = newToastObject.counter.toString();
                if (newToastObject.counter >= 100) {
                    newToast.style.display = 'none';
                    clearInterval(newToastObject.timer);
                    this.toasts = this.toasts.filter((toast) => {
                        return toast.id === newToastObject.id
                    })
                }
            }, 10)
        }
        newToast.addEventListener('click', () => {
            newToast.style.display = 'none';
            clearInterval(newToastObject.timer);
            this.toasts = this.toasts.filter((toast) => {
                return toast.id === newToastObject.id
            })
        });
        this.toasts.push(newToastObject);
        this.toastsContainer.appendChild(newToast);
        return this.id++
    }
}

function toast_success(text) {
    var toasts = new ToastManager();
    toasts.showSuccess(text)
}

function toast_error(text) {
    var toasts = new ToastManager();
    toasts.showError(text)
}

function toast_info(text) {
    var toasts = new ToastManager();
    toasts.showInfo(text)
}

function toast_warning(text) {
    var toasts = new ToastManager();
    toasts.showWarning(text)
}

on('body', 'click', '#logg', function() {
    _this = this; 
    form = _this.parentElement.parentElement.parentElement;
    response = form.parentElement.querySelector(".api_response");
  
    if (!form.querySelector("#id_username").value){
      form.querySelector("#id_username").style.border = "1px #FF0000 solid";
      response.innerHTML = "Введите логин!";
      response.classList.add("error");
      return
    }
    else if (!form.querySelector("#id_password").value){
      form.querySelector("#id_password").style.border = "1px #FF0000 solid";
      response.innerHTML = "Введите пароль!";
      response.classList.add("error")
      return
    }
    else {
      _this.disabled = true;
    }
  
    form_data = new FormData(form);
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/login/", true );
    //link.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
  
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      setCookie("userrr", link.responseText, 300);
      window.location.href = "/"
      }
    else {
      _this.disabled = false;
      response.style.display = "block";
      //response.innerHTML = "Логин или пароль - неверный!";
      response.classList.add("error");
      form.querySelector("#id_username").style.display = "block";
      form.querySelector("#id_username").value = '';
      form.querySelector("#id_password").value = '';
    }};
    link.send(form_data);
  });
  
  on('body', 'click', '#signup', function() {
    _this = this;
    form = _this.parentElement.parentElement.parentElement;
    response = form.parentElement.querySelector(".api_response");
    console.log(form.querySelector("#id_username"));
    if (!form.querySelector("#id_username").value){
      form.querySelector("#id_username").style.border = "1px #FF0000 solid";
      toast_error("Логин - обязательное поле!");
      return
    } else if (!form.querySelector("#id_password").value){
      form.querySelector("#id_password").style.border = "1px #FF0000 solid";
      toast_error("Пароль - обязательное поле!");
      return
    }
    else if (form.querySelector("#id_password").value != form.querySelector("#id_password2").value){
      form.querySelector("#id_password").style.border = "1px #FF0000 solid";
      form.querySelector("#id_password2").style.border = "1px #FF0000 solid";
      toast_error("Пароль - обязательное поле!");
      return
    }
    else {
      form.querySelector("#id_password").style.border = "unset";
      form.querySelector("#id_password2").style.border = "unset";
      form.querySelector("#id_username").style.border = "unset";
      this.disabled = true
    }

    form.querySelector("#signup").setAttribute("disabled", "true");
  
    form_data = new FormData(form);
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/signup/", true );
  
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      setCookie("userrr", link.responseText, 300);
      window.location.href = "/"
      }
    else {
      _this.disabled = false;
      response.style.display = "block";
      response.innerHTML = "not ok";
      response.classList.add("error");
    }};
    link.send(form_data);
  });

 
on('body', 'click', '.search_deceaseds', function() {
  form = this.parentElement.parentElement.parentElement;

  form.querySelector("#id_last_name").style.setProperty('border', 'inherit', 'important');
 
  block = form.nextElementSibling;
  block.innerHTML = "";
  if (!form.querySelector("#id_last_name").value) {
    toast_info("Фамилия - обязательное поле");
    form.querySelector("#id_last_name").style.border = "1px #FF0000 solid";
    return
  }

  url = "/main_search?last_name=" + form.querySelector("#id_last_name").value;
  if (form.querySelector("#id_first_name").value) {
    url += "&first_name=" + form.querySelector("#id_first_name").value
  };
  if (form.querySelector("#id_middle_name").value) {
    url += "&middle_name=" + form.querySelector("#id_middle_name").value
  };
  if (form.querySelector("#id_birth_date").value) {
    url += "&birth_date=" + form.querySelector("#id_birth_date").value
  };
  if (form.querySelector("#id_death_date").value) {
    url += "&death_date=" + form.querySelector("#id_death_date").value
  };
  if (form.querySelector("#id_location").value) {
    url += "&location=" + form.querySelector("#id_location").value
  };

  var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', url, true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link.onreadystatechange = function () {
    if ( this.readyState == 4 && this.status == 200 ) {
        console.log("success!");
        block.innerHTML = link.responseText;
    } else { console.log("status", this.status);  };
  };
  link.send( null );
});


on('body', 'change', '.load_region_items', function() {
  var val = this.value; 
  block = this.parentElement.nextElementSibling;
  if (block.classList.contains("no_region_items")) {
    return;
  }
  if (val == '') {
    block.innerHTML = "";
  } else {
    var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'GET', "/load_region_geo_items/" + val + "/", true );
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function () {
      if ( link.readyState == 4 ) { 
          if ( link.status == 200 ) {
              block.innerHTML = link.responseText;
          }
      }
  };
  link.send( null );
  };
});

on('body', 'change', '.load_regions', function() {
  var val = this.value; 
  block = this.parentElement.nextElementSibling;
  if (!block.classList.contains("load_items")) {
    return;
  }
  if (val == '') {
    block.innerHTML = "";
  } else {
      var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      link.open( 'GET', "/load_regions/" + val + "/", true );
      link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      link.onreadystatechange = function () {
        if ( link.readyState == 4 ) { 
            if ( link.status == 200 ) {
                block.innerHTML = link.responseText;
            }
        }
    };
    link.send( null );
  };
});

on('body', 'click', '.show_dop_search_form', function() {
  block = this.parentElement.parentElement.parentElement.parentElement.nextElementSibling;
  if (block.classList.contains("hidden")) {
    block.classList.remove("hidden")
  } 
  else {
    block.classList.add("hidden");
  }
});
on('body', 'click', '#create_organization', function() {
  let form = this.parentElement.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_name").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_description").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_director").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_phone").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название организации");
      return
  }
  else if (!form.querySelector("#id_description").value) {
    form.querySelector("#id_description").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите описание организации");
    return
  }
  else if (!form.querySelector("#id_director").value) {
    form.querySelector("#id_director").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите директора организации");
    return
  }
  else if (!form.querySelector("#id_phone").value) {
    form.querySelector("#id_phone").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите телефон организации");
    return
  }

  form.querySelector("#create_organization").setAttribute("disabled", "true");
  form.querySelector("#create_organization").innerHTML = "Идет сохранение";

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_organization/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      window.location.href = '/organization/' + link.responseText + "/";
    }};
    link.send(form_data);
});

on('body', 'click', '#create_place', function() {
  let form = this.parentElement.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_title").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_country").style.setProperty('border', 'inherit', 'important');

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

on('body', 'click', '#create_deceased', function() { 
  let form = this.parentElement.parentElement.parentElement;
  form.querySelector("#id_first_name").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_last_name").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_birth_date").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_death_date").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#place_id").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#get_my_lat").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#get_my_lon").style.setProperty('border', 'inherit', 'important');

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

on('body', 'click', '#edit_organization', function() {
  _this = this;
  form = _this.parentElement.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_name").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_description").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_director").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_phone").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название организации");
      return
  }
  else if (!form.querySelector("#id_description").value) {
    form.querySelector("#id_description").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите описание организации");
    return
  }
  else if (!form.querySelector("#id_director").value) {
    form.querySelector("#id_director").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите директора организации");
    return
  }
  else if (!form.querySelector("#id_phone").value) {
    form.querySelector("#id_phone").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите телефон организации");
    return
  }

  form.querySelector("#edit_organization").setAttribute("disabled", "true");
  form.querySelector("#edit_organization").innerHTML = "Идет сохранение";
  
  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/edit_organization/" + _this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      window.location.href = '/organization/' + link.responseText + "/";
    }};
    link.send(form_data);
});

on('body', 'click', '#create_loc', function() {
  let form = this.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#country_id").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_address2").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#country_id").value) {
      form.querySelector("#country_id").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите страну организации");
      return
  }
  else if (!form.querySelector("#id_address2").value) {
    form.querySelector("#id_address2").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите улицу и дом организации");
    return
  }

  form.querySelector("#create_loc").setAttribute("disabled", "true");
  form.querySelector("#create_loc").innerHTML = "Идет сохранение";

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_loc/" + this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      window.location.href = '/organization/' + link.responseText + "/";
    }};
    link.send(form_data);
});

on('body', 'click', '#edit_loc', function() {
  _this = this;
  form = _this.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_country").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_address2").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_country").value) {
    form.querySelector("#id_country").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите страну организации");
    return
  }
  else if (!form.querySelector("#id_address2").value) {
    form.querySelector("#id_address2").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите улицу и дом организации");
    return
  }

  form.querySelector("#edit_loc").setAttribute("disabled", "true");
  form.querySelector("#edit_loc").innerHTML = "Идет сохранение";
  
  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/edit_loc/" + _this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      window.location.href = '/organization/' + link.responseText + "/";
    }};
    link.send(form_data);
});

on('body', 'click', '#create_service', function() {
  let form = this.parentElement.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_price").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_description").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_title").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_price").value) {
      form.querySelector("#id_price").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите цену услуги");
      return
  }
  else if (!form.querySelector("#id_description").value) {
    form.querySelector("#id_description").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите описание услуги");
    return
  }
  else if (!form.querySelector("#id_title").value) {
    form.querySelector("#id_title").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите название услуги");
    return
  }

  form.querySelector("#create_service").setAttribute("disabled", "true");
  form.querySelector("#create_service").innerHTML = "Идет сохранение";

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_service/" + this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      window.location.href = '/organization/' + link.responseText + "/";
    }}; 
    link.send(form_data);
});

on('body', 'click', '#edit_service', function() {
  _this = this;
  form = _this.parentElement.parentElement.parentElement.parentElement.parentElement;
  
  form.querySelector("#id_price").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_description").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_title").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_price").value) {
      form.querySelector("#id_price").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите цену услуги");
      return
  }
  else if (!form.querySelector("#id_description").value) {
    form.querySelector("#id_description").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите описание услуги");
    return
  }
  else if (!form.querySelector("#id_title").value) {
    form.querySelector("#id_title").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите название услуги");
    return
  }

  form.querySelector("#edit_service").setAttribute("disabled", "true");
  form.querySelector("#edit_service").innerHTML = "Идет сохранение";
  
  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/edit_service/" + _this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      window.location.href = '/organization/' + link.responseText + "/";
    }};
    link.send(form_data);
});


function delete_item_2(url, id) {
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

on('body', 'click', '.delete_service', function() {
  delete_item("/delete_service/", this.getAttribute("data-pk"));
  this.parentElement.parentElement.parentElement.remove();
});
on('body', 'click', '.delete_loc', function() {
  delete_item("/delete_loc/", this.getAttribute("data-pk"));
  this.parentElement.parentElement.remove();
});

on('body', 'click', '#images_container', function() {
  this.previousElementSibling.click();
});

on('body', 'change', '#pro-images', function() {
  len = this.files.length;
  if (len > 10) {
    alert("Максимальное количество фотографий - 10")
  }

  a = len % 10;
  b = len % 100;

  if (a == 1 && b != 11) {
    word = "Выбранa " + len + " фотография";
  }
  else if (a >= 2 && a <= 4 && (b < 10 || b >= 20)) {
    word = "Выбраны " + len + " фотографии";
  }
  else {
    word = "Выбраны " + len + " фотографий";
  }

  document.body.querySelector(".photos_upload_response").innerHTML = word;
}); 

on('body', 'click', '.remove_file', function() {
  delete_item("/delete_file/", this.getAttribute("data-pk"));
  this.parentElement.remove();
}); 