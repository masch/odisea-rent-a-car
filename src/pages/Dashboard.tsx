import { useEffect, useState } from "react";
import { CarsList } from "../components/CarList";
import { CreateCarForm } from "../components/CreateCarForm";
import { SetAdminFeeModal } from "../components/SetAdminFeeModal";
import StellarExpertLink from "../components/StellarExpertLink";
import useModal from "../hooks/useModal";
import { ICar } from "../interfaces/car";
import { CarStatus } from "../interfaces/car-status";
import { IRentACarContract } from "../interfaces/contract";
import { CreateCar } from "../interfaces/create-car";
import { SetAdminFee } from "../interfaces/set-admin-fee";
import { UserRole } from "../interfaces/user-role";
import { useStellarAccounts } from "../providers/StellarAccountProvider";
import { stellarService } from "../services/stellar.service";
import { walletService } from "../services/wallet.service";
import { ONE_XLM_IN_STROOPS } from "../utils/xlm-in-stroops";

export default function Dashboard() {
  const { hashId, cars, walletAddress, setCars, setHashId, selectedRole } =
    useStellarAccounts();
  const {
    showModal: showModalCreateCar,
    openModal: openModalCreateCar,
    closeModal: closeModalCreateCar,
  } = useModal();
  const {
    showModal: showModalSetAdminFee,
    openModal: openModalSetAdminFee,
    closeModal: closeModalSetAdminFee,
  } = useModal();

  const [availableAdminFeeToWthdraw, setAvailableAdminFeeToWthdraw] =
    useState(0);

  useEffect(() => {
    if (selectedRole !== UserRole.ADMIN) return;

    const loadAdminFreeToWithdraw = async () => {
      const contractClient =
        await stellarService.buildClient<IRentACarContract>(walletAddress);

      const { result: available_to_withdraw } =
        await contractClient.get_admin_fee_to_withdraw();

      setAvailableAdminFeeToWthdraw(Number(available_to_withdraw));
    };

    void loadAdminFreeToWithdraw();
  }, [selectedRole, walletAddress]);

  const handleWithdrawFee = async () => {
    const contractClient =
      await stellarService.buildClient<IRentACarContract>(walletAddress);

    const withdrawAdminFeeResult = await contractClient.withdraw_admin_fee();
    const xdr = withdrawAdminFeeResult.toXDR();

    const signedTx = await walletService.signTransaction(xdr);
    const txHash = await stellarService.submitTransaction(signedTx.signedTxXdr);

    setAvailableAdminFeeToWthdraw(0);
    setHashId(txHash as string);
  };

  const handleSetAdminFee = async (formData: SetAdminFee) => {
    const contractClient =
      await stellarService.buildClient<IRentACarContract>(walletAddress);

    const setAdminFeeResult = await contractClient.set_admin_fee({
      admin_fee: formData.amount * ONE_XLM_IN_STROOPS,
    });
    const xdr = setAdminFeeResult.toXDR();

    const signedTx = await walletService.signTransaction(xdr);
    const txHash = await stellarService.submitTransaction(signedTx.signedTxXdr);

    setHashId(txHash as string);
    closeModalSetAdminFee();
  };

  const handleCreateCar = async (formData: CreateCar) => {
    const { brand, model, color, passengers, pricePerDay, ac, ownerAddress } =
      formData;
    const contractClient =
      await stellarService.buildClient<IRentACarContract>(walletAddress);

    const addCarResult = await contractClient.add_car({
      owner: ownerAddress,
      price_per_day: pricePerDay * ONE_XLM_IN_STROOPS,
    });
    const xdr = addCarResult.toXDR();

    const signedTx = await walletService.signTransaction(xdr);
    const txHash = await stellarService.submitTransaction(signedTx.signedTxXdr);

    const newCar: ICar = {
      brand,
      model,
      color,
      passengers,
      pricePerDay,
      ac,
      ownerAddress,
      status: CarStatus.AVAILABLE,
    };

    setCars((prevCars) => [...prevCars, newCar]);
    setHashId(txHash as string);
    closeModalCreateCar();
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold" data-test="dashboard-title">
          Cars Catalog
        </h1>
        {selectedRole === UserRole.ADMIN && (
          <div className="flex gap-4">
            <button
              onClick={openModalSetAdminFee}
              className="group px-6 py-3 bg-indigo-600 text-white font-semibold rounded-xl shadow-lg hover:bg-indigo-700 hover:shadow-xl disabled:bg-slate-300 disabled:cursor-not-allowed transition-all duration-200 transform hover:scale-105 disabled:transform-none cursor-pointer"
            >
              <span className="flex items-center gap-2">Admin fee</span>
            </button>
            <button
              onClick={() => {
                void handleWithdrawFee();
              }}
              disabled={availableAdminFeeToWthdraw <= 0}
              className="group px-6 py-3 bg-indigo-600 text-white font-semibold rounded-xl shadow-lg hover:bg-indigo-700 hover:shadow-xl disabled:bg-slate-300 disabled:cursor-not-allowed transition-all duration-200 transform hover:scale-105 disabled:transform-none cursor-pointer"
            >
              <span className="flex items-center gap-2">Withdraw fee</span>
            </button>

            <button
              onClick={openModalCreateCar}
              className="group px-6 py-3 bg-indigo-600 text-white font-semibold rounded-xl shadow-lg hover:bg-indigo-700 hover:shadow-xl disabled:bg-slate-300 disabled:cursor-not-allowed transition-all duration-200 transform hover:scale-105 disabled:transform-none cursor-pointer"
            >
              <span className="flex items-center gap-2">Add Car</span>
            </button>
          </div>
        )}
      </div>

      {cars && <CarsList cars={cars} />}

      {showModalSetAdminFee && (
        <SetAdminFeeModal
          onSetAdminFee={handleSetAdminFee}
          onCancel={closeModalSetAdminFee}
        />
      )}

      {showModalCreateCar && (
        <CreateCarForm
          onCreateCar={handleCreateCar}
          onCancel={closeModalCreateCar}
        />
      )}
      {hashId && <StellarExpertLink url={hashId} />}
    </div>
  );
}
