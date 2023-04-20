import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import {AccueilComponent} from "./accueil/accueil.component";
import {EmailFormulaireComponent} from "./email-formulaire/email-formulaire.component";
import {RevokeFormulaireComponent} from "./revoke-formulaire/revoke-formulaire.component";
import {ConfirmationMailComponent} from "./confirmation-mail/confirmation-mail.component";
import {FormDownloadComponent} from "./form-download/form-download.component";



const routes: Routes = [
  { path: '', redirectTo: '/accueil', pathMatch: 'full' },
  { path: 'accueil', component: AccueilComponent },
  { path: 'form-email', component: EmailFormulaireComponent },
  { path: 'form-revoke', component: RevokeFormulaireComponent },
  { path: 'confirmation-mail', component: ConfirmationMailComponent },
  { path:'form-download', component: FormDownloadComponent  }



];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
