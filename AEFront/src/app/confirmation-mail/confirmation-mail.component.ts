import { Component, OnInit } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Router} from "@angular/router";
import {RequestService} from "../request.service";

@Component({
  selector: 'app-confirmation-mail',
  templateUrl: './confirmation-mail.component.html',
  styleUrls: ['./confirmation-mail.component.scss']
})
export class ConfirmationMailComponent implements OnInit {

  constructor(private http: HttpClient, private router: Router, private request: RequestService) { }

    ngOnInit() {
      let a: String
      a=""
      let x;
      for (let i = 0; i < 6; i++) {
        x = this.getRandomInt(10)
        //convert x to string and concatenate it to a
        a = a + x.toString()
      }
      console.log(a)
    }

    getRandomInt(max: number) {
      return Math.floor(Math.random() * max);
    }

  submitForm() {
    const otp = (<HTMLInputElement>document.getElementById('otp')).value;
    const formData = new FormData();
    formData.append('OTP', otp);
    const uploadRes = this.http.post('http://localhost:8740/api/csr/validation', formData);
    uploadRes.subscribe((res: any) => {
      console.log(res);
      if (res.status=="ok"){
        this.loadurl(res)
      }
    });

  }

  loadurl(res: any){
    this.request.current_certificate_id = res.cert_id
    this.request.revokation_OTP = res.otp_revokation
    this.router.navigate(['/form-download'])
  }
}