import {Component, Input} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Router} from "@angular/router";

@Component({
  selector: 'app-email-formulaire',
  templateUrl: './email-formulaire.component.html',
  styleUrls: ['./email-formulaire.component.scss']
})
export class EmailFormulaireComponent {
  constructor(private http: HttpClient, private router: Router) {
  }
  srcResult: any;

  onFileSelected() {
    const inputNode: any = document.querySelector('#file');
    if (typeof (FileReader) !== 'undefined') {
      const reader = new FileReader();

      reader.onload = (e: any) => {
        this.srcResult = e.target.result;
      };
      reader.readAsArrayBuffer(inputNode.files[0]);
    }
  }

  submitForm() {
    const inputNode: any = document.querySelector('#file');
    const inputNode2: any = document.querySelector('#email');
    const file = inputNode.files[0];
    console.log(inputNode2);
    const email = inputNode2.value;
    const formData = new FormData();
    formData.append('csr', file, file.name);
    formData.append('email', email);
    const uploadRes = this.http.post('http://192.168.16.42:8740/api/csr/request', formData);
    uploadRes.subscribe((res: any) => {
      console.log(res)

      if (res.status=="ok")
        this.loadurl();

    });
  }

  loadurl(){
    this.router.navigate(['/confirmation-mail'])
  }
}
