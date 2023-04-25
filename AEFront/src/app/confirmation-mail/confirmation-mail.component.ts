import {Component, OnInit} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Router} from "@angular/router";
import {RequestService} from "../request.service";

@Component({
  selector: 'app-confirmation-mail',
  templateUrl: './confirmation-mail.component.html',
  styleUrls: ['./confirmation-mail.component.scss']
})
export class ConfirmationMailComponent implements OnInit {

  constructor(private http: HttpClient, private router: Router, private request: RequestService) {
  }

  email: string = "";

  ngOnInit() {
    this.email = history.state.email || "";
  }

  getRandomInt(max: number) {
    return Math.floor(Math.random() * max);
  }

  submitForm() {
    const otp = (<HTMLInputElement>document.getElementById('otp')).value;
    const body = {
      otp: otp,
      email: this.email
    }
    const uploadRes = this.http.post('http://localhost:8740/api/csr/validation', body, {observe: 'response'});
    uploadRes.subscribe((res: any) => {
      console.log(res);
      if (res.status === 200) {
        this.loadurl(res.body)
      }
    });

  }

  loadurl(res: any) {
    this.request.current_certificate_id = res.crt_id
    this.request.revokation_OTP = res.otp_revok
    this.router.navigate(['/form-download'])
  }
}
