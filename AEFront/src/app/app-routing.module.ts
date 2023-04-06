import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import {AccueilComponent} from "./accueil/accueil.component";
import {EmailFormulaireComponent} from "./email-formulaire/email-formulaire.component";
import {RevokeFormulaireComponent} from "./revoke-formulaire/revoke-formulaire.component";
import {ConfirmationMailComponent} from "./confirmation-mail/confirmation-mail.component";



const routes: Routes = [
  { path: '', component: AccueilComponent },
  { path: 'accueil', component: AccueilComponent },
  { path: 'form-email', component: EmailFormulaireComponent },
  { path: 'form-revoke', component: RevokeFormulaireComponent },
  { path: 'confirmation-mail', component: ConfirmationMailComponent }



];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
