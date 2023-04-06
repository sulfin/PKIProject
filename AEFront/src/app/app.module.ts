import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { EmailFormulaireComponent } from './email-formulaire/email-formulaire.component';
import { AccueilComponent } from './accueil/accueil.component';
import {MatIconModule} from "@angular/material/icon";
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { RevokeFormulaireComponent } from './revoke-formulaire/revoke-formulaire.component';
import {MatInputModule} from "@angular/material/input";
import { ConfirmationMailComponent } from './confirmation-mail/confirmation-mail.component';

@NgModule({
  declarations: [
    AppComponent,
    EmailFormulaireComponent,
    AccueilComponent,
    RevokeFormulaireComponent,
    ConfirmationMailComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    MatIconModule,
    BrowserAnimationsModule,
    MatInputModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
