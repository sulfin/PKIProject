import { Component } from '@angular/core';
import {HttpClient} from "@angular/common/http";

@Component({
  selector: 'app-revoke-formulaire',
  templateUrl: './revoke-formulaire.component.html',
  styleUrls: ['./revoke-formulaire.component.scss']
})
export class RevokeFormulaireComponent {
  constructor(private http: HttpClient) {

}

  submitForm() {
    const inputNode: any = document.querySelector('#certid');
    const inputNode2: any = document.querySelector('#email');
    const inputNode3: any = document.querySelector('#otp');
    const certid = inputNode.value;
    const email = inputNode2.value;
    const otp = inputNode3.value;
    const formData = new FormData();
    formData.append('certid', certid);
    formData.append('email', email);
    formData.append('otp', otp);
    const uploadRes = this.http.post('http://192.168.16.42:8740/api/crt/revoke', formData);
    console.log(formData)
    uploadRes.subscribe((res: any) => {
      console.log(res)

      if (res.status=="ok")
        console.log("revocation request sent")

    });
  }

  }

