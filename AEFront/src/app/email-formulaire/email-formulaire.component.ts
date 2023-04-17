import { Component } from '@angular/core';
import {HttpClient} from "@angular/common/http";

@Component({
  selector: 'app-email-formulaire',
  templateUrl: './email-formulaire.component.html',
  styleUrls: ['./email-formulaire.component.scss']
})
export class EmailFormulaireComponent {
  constructor(private http: HttpClient) {
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
    const file = inputNode.files[0];
    const formData = new FormData();
    formData.append('file', file, file.name);
    formData.append('email', '');
    const uploadRes = this.http.post('http://localhost:8740/api/csr/request', formData);
    uploadRes.subscribe((res) => {
      console.log(res);
    });
  }
}
