import { Component } from '@angular/core';
import {RequestService} from "../request.service";

@Component({
  selector: 'app-form-download',
  templateUrl: './form-download.component.html',
  styleUrls: ['./form-download.component.scss']
})
export class FormDownloadComponent  {

  constructor(public request: RequestService) {
  }




}
