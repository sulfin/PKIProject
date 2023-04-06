import { Component } from '@angular/core';

@Component({
  selector: 'app-email-formulaire',
  templateUrl: './email-formulaire.component.html',
  styleUrls: ['./email-formulaire.component.scss']
})
export class EmailFormulaireComponent {
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

}
