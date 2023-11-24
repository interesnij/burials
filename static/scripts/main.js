function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};

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

on('body', 'click', '.open_child_serves', function(event) {
  if (event.target.classList.contains("get_serve_info")) {
    return
  };
  parent_id = this.getAttribute("parent-pk");
  check = this.querySelector(".icon_parent");
  if (check.innerHTML == "▼") {
    check.innerHTML = "▲"
  }
  else {
    check.innerHTML = "▼"
  }
  childs = this.parentElement.querySelectorAll('[serve-pk=' + '"' + parent_id + '"' + ']');
  for (var i = 0; i < childs.length; i++) {
    if (childs[i].classList.contains("select_child_serve")) {
      childs[i].classList.toggle("hide");
    }
  }
});

on('body', 'click', '#logg', function() {
    _this = this; 
    form = _this.parentElement.parentElement.parentElement;
    response = form.querySelector(".api_response");
  
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
    username = form.querySelector("#id_username");
    response = form.querySelector(".api_response");
    if (!username.value){
      username.style.border = "1px #FF0000 solid";
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
  
    form_data = new FormData(form);
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/signup/", true );
  
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
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


  on('body', 'input', '.desctop_search', function() {
    _this = this;
    _help = _this.previousElementSibling;
    value = _this.value;
    parent = _this.parentElement.parentElement.parentElement.parentElement.parentElement;
    content_block = parent.querySelector(".content_block");
    search_block = content_block.previousElementSibling;

    if (value == "") {
      search_block.innerHTML= "";
      content_block.classList.remove("hidden");
      return;
    }
    else if (value.length < 3) {
      search_block.innerHTML= "";
      content_block.classList.remove("hidden");
      return;
    }

    if (_this.getAttribute("data-folder")) {
      folder = _this.getAttribute("data-folder")
    } else {
      folder = ""
    };
    url = "/search/" + value + "/";

    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', url + "?ajax=1", true );
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        elem_ = document.createElement('span');
        elem_.innerHTML = ajax_link.responseText;

        elem_.querySelector(".is_paginate") ?
        (
          search_section = elem_.querySelector(".is_paginate"),
          search_block.innerHTML = search_section.innerHTML.replaceAll(new RegExp(value, 'ig'), "<span class='selected'>" + value + "</span>")
        ) : search_block.innerHTML = "<div style='margin-top: 40px;'><div class='align-center'><span class='border' style='padding: 10px 15px;'>Искать пока не из чего...</div></div>";
        content_block.classList.add("hidden")
      }
    }
    ajax_link.send();
});