import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";

@Injectable({
  providedIn: 'root'
})
export class RequestService {
  baseurl = 'http://localhost:8740/api';
  certificate_base='/crt/crt/'
  full_chain_base='/crt/fullchain/'
  root_ca_base='/crt/rootca.crt'
  current_certificate_id = '';
  revokation_OTP = '';

  public getchainedurl() {
    return this.baseurl + this.full_chain_base + this.current_certificate_id;
  }

  public getcrturl() {
    return this.baseurl + this.certificate_base + this.current_certificate_id + ".crt";
  }

  public getrootcaurl() {
    return this.baseurl + this.root_ca_base;
  }


  constructor(private http: HttpClient) {
  }
}
